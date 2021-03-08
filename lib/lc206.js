/// 反转链表
/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
export var reverseList = function (head) {
    let [p1, p2] = [null, head]
    while (p2) {
        let p3 = p2.next
        p2.next = p1
        p1 = p2
        p2 = p3
    }
    return p1
};