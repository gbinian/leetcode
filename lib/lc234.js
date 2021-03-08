/// 判断一个链表是否为回文链表。
import { reverseList } from './lc206'
/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {boolean}
 */
export var isPalindrome = function(head) {
    let p = head
    let arr = []
    // 使用数组
    while (p) {
        arr.push(p.val)
        p = p.next
    }
    for (let i = 0; i < arr.length / 2; i++) {
        if (arr[i] !== arr[arr.length - 1 - i]) return false
    }
    return true
};