/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
export var mergeTwoLists = function(l1, l2) {
    let current = null;
    let head = null
    while (l1 && l2) {
        if (l1.val > l2.val) {
            if (current) {
                current.next = l2;
                current = current.next
            } else {
                current = l2
            }
            l2 = l2.next
        } else {
            if (current) {
                current.next = l1;
                current = current.next
            } else {
                current = l1
            }
            l1 = l1.next
        }
        if (!head) head = current
        current.next = null
    }
    if (l1) current ? current.next = l1 : head = l1;
    if (l2) current ? current.next = l2 : head = l2;
    return head
};