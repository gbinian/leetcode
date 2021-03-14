/**
* Definition for singly-linked list.
* function ListNode(val) {
*     this.val = val;
*     this.next = null;
* }
*/

/**
* @param {ListNode} head
* @return {boolean}
*/
export var hasCycle = function (head) {
    if (!head) return false
    let slow = head
    let fast
    if (slow.next) {
        fast = slow.next
    } else {
        return false
    }
    while (slow !== fast) {
        if (slow === null || fast === null) {
            return false
        }
        slow = slow.next
        if (fast.next) {
            fast = fast.next.next
        } else {
            return false
        }
        
    }
    return true
};