import { ListNode, generateListNodes } from './utils'
import { isPalindrome } from './lc234'


test('lc234-回文链表', () => {
    let l1 = generateListNodes([1, 2, 2, 1])
    expect(true).toEqual(isPalindrome(l1))
})