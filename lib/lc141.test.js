import { ListNode, generateListNodes } from './utils'

import { hasCycle } from './lc141'

test('lc141-环形链表', () => {
    let l1 = generateListNodes([1, 2, 3, 4, 5])
    l1.next.next.next.next.next = l1.next.next
    expect(true).toBe(hasCycle(l1));
})