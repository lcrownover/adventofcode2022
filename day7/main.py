#!/usr/bin/env python3


# gonna do this one in python and port it once i'm done
# since python is better for pseudocode

from typing import Literal, Union, List


class Node:
    def __init__(
        self, name: str, type: Union[Literal["file"], Literal["directory"]], size: int
    ):
        self.name = name
        self.type = type
        self.size = int(size)
        self.children = []
        self.total_size = 0

    def set_id(self, id: int):
        self.id = id

    def set_total_size(self):
        self.total_size = self.get_recurse_size()

    def set_parent_id(self, id: int):
        self.parent_id = id

    def get_children(self) -> List:
        return self.children

    def get_recurse_size(self) -> int:
        size = 0
        for child in self.get_children():
            size += child.size
            size += child.get_recurse_size()
        return size

    def get_recurse_size_le(self, le_size: int) -> int:
        global recursive_size
        for c in self.get_children():
            csize = c.get_recurse_size()
            if 0 < csize <= le_size:
                recursive_size += csize
            c.get_recurse_size_le(le_size)
        return recursive_size

    def display(self, indent: int = 0):
        s = " " * indent
        print(f"{s}- {self.name} ({self.type}, size={self.size})")
        for c in self.children:
            c.display(indent + 2)


class NodeNotFoundError(Exception):
    def __init__(self, msg: str = ""):
        super().__init__(msg)


class Filesystem:
    def __init__(self):
        self.nodes = {0: Node(name="/", type="directory", size=0)}

    def get_node_by_id(self, node_id: int) -> Node:
        if node_id in self.nodes:
            return self.nodes[node_id]
        raise NodeNotFoundError

    def get_next_id(self) -> int:
        return len(self.nodes) + 1

    def get_child_id_by_name_and_parent_id(self, name: str, parent_id: int) -> int:
        for c in self.get_node_by_id(parent_id).get_children():
            if c.name == name:
                return c.id
        raise NodeNotFoundError()

    def add_node(self, node: Node, parent_id: int) -> int:
        # Register the node
        node.id = self.get_next_id()
        node.set_parent_id(parent_id)
        self.nodes[self.get_next_id()] = node
        # Link it to its parent
        # Since python is reference based,
        # this shouldn't be much additional memory?
        self.nodes[parent_id].children.append(node)
        return node.id

    def get_recurse_size(self) -> int:
        return self.nodes[0].get_recurse_size()

    def get_recurse_size_le(self, le_size: int) -> int:
        global recursive_size
        recursive_size = 0
        return self.nodes[0].get_recurse_size_le(le_size=le_size)

    def display(self):
        self.nodes[0].display()


def create_filesystem(input_data) -> Filesystem:
    fs = Filesystem()

    cursor = 0  # parent_id of root

    for line in input_data:

        # print(f"=> {line}")

        # parsing commands, either "cd" or "ls"
        if line.startswith("$ cd"):
            destination = line.split(" ")[2]
            # destination can be either "/", "..", or some child dir
            if destination == "/":
                cursor = 0
            elif destination == "..":
                cursor = fs.get_node_by_id(cursor).parent_id
            else:
                cursor = fs.get_child_id_by_name_and_parent_id(destination, cursor)
            continue

        # print(f"cursor:{cursor}")

        if line.startswith("$ ls"):
            continue

        if line.startswith("dir"):
            dirname = line.split(" ")[1]
            # print(f"adding dir: {dirname}")
            fs.get_node_by_id(cursor)
            n = Node(name=dirname, type="directory", size=0)
            fs.add_node(n, parent_id=cursor)
            continue

        # add the files
        size = line.split(" ")[0]
        filename = line.split(" ")[1]
        # print(f"adding file: {filename}")
        n = Node(name=filename, type="file", size=size)
        fs.add_node(n, cursor)

    return fs


def part1():
    # with open("example.txt", "r") as f:
    with open("input.txt", "r") as f:
        data = [line.strip() for line in f.readlines()]

    fs = create_filesystem(data)
    # fs.display()

    print(fs.get_recurse_size_le(100000))


def part2():
    # with open("example.txt", "r") as f:
    with open("input.txt", "r") as f:
        data = [line.strip() for line in f.readlines()]

    fs = create_filesystem(data)

    node_list = list(fs.nodes.values())
    for node in node_list:
        node.set_total_size()

    sorted_node_list = sorted(node_list, key=lambda v: v.total_size)
    total_size = 70000000
    update_size = 30000000
    threshold_size = total_size - update_size

    used_size = sorted_node_list[-1].total_size
    print(f"current used size: {sorted_node_list[-1].name} {used_size}")

    # just want to be done with it
    fitted_sizes = []
    for node in sorted_node_list:
        if (used_size - node.total_size) <= threshold_size:
            # print(node.total_size)
            fitted_sizes.append(node.total_size)
    print(fitted_sizes[0])





# part1()
part2()
