//
// This is only a SKELETON file for the 'Zipper' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

function deepClone(obj) {
  return JSON.parse(JSON.stringify(obj));
}

export class Zipper {
  constructor(node, parent = null) {
    this.node = deepClone(node);
    this.parent = deepClone(parent);
  }

  static fromTree(tree) {
    if (tree === null) return null;
    return new Zipper(tree);
  }

  toTree() {
    if (!this.parent) return this.node;
    return this.up().toTree();
  }

  value() {
    return this.node.value;
  }

  left() {
    if (this.node.left === null) return null;

    return new Zipper(this.node.left, {
      parent: this.parent,
      value: this.node.value,
      otherChild: this.node.right,
      nodeIsLeftOfParent: true,
    });
  }

  right() {
    if (this.node.right === null) return null;
    return new Zipper(this.node.right, {
      parent: this.parent,
      value: this.node.value,
      otherChild: this.node.left,
      nodeIsLeftOfParent: false,
    });
  }

  up() {
    if (!this.parent) return null;
    const { nodeIsLeftOfParent, value } = this.parent;
    return new Zipper(
      {
        value,
        left: nodeIsLeftOfParent ? this.node : this.parent.otherChild,
        right: nodeIsLeftOfParent ? this.parent.otherChild : this.node,
      },
      this.parent.parent
    );
  }

  setValue(value) {
    this.node.value = value;
    return new Zipper(this.node, this.parent);
  }

  setLeft(node) {
    this.node.left = node;
    return new Zipper(this.node, this.parent);
  }

  setRight(node) {
    this.node.right = node;
    return new Zipper(this.node, this.parent);
  }
}
