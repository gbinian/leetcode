/**
 * 二叉树的最大深度
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number}
 */
export var maxDepth = function(root) {
    if (root === null) return 0;
    let [left, right] = [0, 0];
    left = maxDepth(root.left)
    right = maxDepth(root.right)
    return ( left > right ? left : right ) + 1
};