/// 给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。
/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @param {number} n
 * @return {ListNode}
 */
export var removeNthFromEnd = function(head, n) {
    let [p1, p2, count] = [null, head, 1];
    while (p2.next != null) {
        p2 = p2.next;
        count += 1;
        if (count > n ) {
            p1 = p1 === null ? head : p1.next
        }
    }
    // 删除p1 下一个节点
    if (p1 && p1.next) {
        p1.next = p1.next.next
        return head
    } else {
        return head.next;
    }
    
};

