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

/**
 *
 * @param {Array<number>} arr
 * @return {TreeNode}
 */
export function generateTreeNodes(arr) {
    if (arr.length === 0 ){
        return null
    }
    const root = new TreeNode(arr[0]);
    let [count, i, endIndex] = [1, 1, 0]; // 当前层数， 当前循环到的数组下标
    let prev = [root];       // 上一层的节点
    for (i = 1; i < arr.length; i++) {
        endIndex = endIndex + Math.pow(2, count); // count层结束位置
        if (i > endIndex) count += 1

    }
    // todo!
    return root
}

