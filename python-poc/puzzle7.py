from __future__ import annotations
import enum
import typing
import re

NodeType = typing.Literal[
    "Dir",
    "File"
]

NodeDetails = typing.TypedDict(
    "NodeDetails",
    {
        "name": str,
        "type": NodeType
    }
)

class Node:

    def __init__(self, name: str) -> None:
        self.name = name
        self.parent: typing.Optional[Node] = None



class Folder(Node):

    def __init__(self, name: str, parent: typing.Optional[Folder]) -> None:
        super().__init__(name)
        self.size: int = 0
        self.sub_folders: typing.Dict[str, Folder] = {}
        self.sub_files: typing.Dict[str, File] = {}
        self.parent: typing.Optional[Folder] = parent


    def add_node(self, child_node: typing.Union[Folder, File]):
        if isinstance(child_node, Folder):
            name: str = child_node.name
            self.sub_folders[name] = child_node
        elif isinstance(child_node, File):
            name: str = child_node.name
            self.sub_files[name] = child_node
        else:
            raise TypeError(f"Expected {Folder} or {File} types, got {type(child_node)} instead.")

    def get_size(self) -> int:
        total_folder_size: int = 0
        for folder in self.sub_folders.values():
            total_folder_size += folder.get_size()
        total_file_size: int = 0
        for file in self.sub_files.values():
            total_file_size += file.get_size()
        return total_file_size + total_folder_size
        
class File(Node):
    
    def __init__(self, name: str, size: int, parent: Folder) -> None:
        super().__init__(name)
        self.size: int = size
        self.parent: typing.Optional[Folder] = parent

    def get_size(self) -> int:
        return self.size



# MAIN

## Preparing the regex patterns
patterns: typing.Dict[str, re.Pattern] = {}
patterns["ls"] = re.compile(r"\$ ls")
patterns["cd"] = re.compile(r"\$ cd ([A-z0-9\/\.]+)")
patterns["dir"] = re.compile(r"dir ([A-z0-9]+)")
patterns["file"] = re.compile(r"([0-9]+) ([A-z0-9]+.[A-z]+)")

## Creating the initial root folder
cursor: Folder = Folder(r"/", None)
with open(r"inputs/input7.txt") as opened:
    for i,line in enumerate(opened):
        if i == 0: # Skipping the initialization of the root folder
            continue
        
        matched: typing.Optional[re.Match]
        
        # Evaluating ls
        matched = patterns["ls"].match(line)
        if matched is not None:
            continue # No need to evaluate ls

        # Evaluating cd
        matched = patterns["cd"].match(line)
        if matched is not None:
            directory: str = matched.group(1)
            if directory == r"/":
                print("Moving to root node.")
                while cursor.parent is not None: # Move up the tree until root node
                    cursor = cursor.parent
            elif directory == r"..": # Moving up a single node
                if cursor.parent is not None:
                    print(f"Moving up to folder {cursor.parent.name}.")
                    cursor = cursor.parent
            else:
                print(f"Moving down to folder {directory}.")
                if directory not in cursor.sub_folders.keys(): # Can only move into folders that exist
                    raise ValueError(f"Current folders: {cursor.sub_folders.keys()}\n {directory} does not exist.")
                else:
                    cursor = cursor.sub_folders[directory]
            continue

        # Evaluating dir
        matched = patterns["dir"].match(line)
        if matched is not None:
            directory: str = matched.group(1)
            print(f"Found directory {directory}.")
            if directory in cursor.sub_folders.keys():
                pass
            else:
                new_folder = Folder(directory, cursor)
                cursor.add_node(new_folder)
            continue

        # Evaluating dir
        matched = patterns["file"].match(line)
        if matched is not None:
            size: int = int(matched.group(1))
            file: str = matched.group(2)
            print(f"Found file {file}.")
            if file in cursor.sub_files.keys():
                pass
            else:
                # new_folder = Folder(directory, cursor)
                new_file = File(file, size=size, parent=cursor)
                cursor.add_node(new_file)
            continue

# Moving to root folder
while cursor.parent is not None: # Move up the tree until root node
    cursor = cursor.parent
total_size = cursor.get_size()
print(f"total_size = {total_size}")