export function ListNode(val) {
    this.val = val;
    this.next = null;
}

export function generateListNodes (arr) {
    let [p, head] = [null, null];
    if(arr.length  === 1) return new ListNode(arr[0])
    for (let i = 0; i < arr.length - 1; i++) {
        if (head === null) head = new ListNode(arr[i]);
        if (p === null) p = head;
        p.next = arr[i + 1] ? new ListNode(arr[i + 1]) : null
        p = p.next
    }
    
    return head
}

export function TreeNode(val, left, right) {
    this.val = (val===undefined ? 0 : val)
    this.left = (left===undefined ? null : left)
    this.right = (right===undefined ? null : right)
}

/// 数组一定是包含一颗完全二叉树的所有节点，按照行的顺序给出节点
export function generateTreeNodes(arr) {
    if(arr.length === 0) return null
    let root = new TreeNode(arr[0]);
    let prev = [root];
    let i = 1; // 下标
    let row = 1; // 当前行数
    while( i < arr.length) {

        prev = []
        prev.push(); // 新节点添加
    }
    return root
}