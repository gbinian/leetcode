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