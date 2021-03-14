import { generateListNodes } from './utils'
import { mergeTwoLists } from './lc21'

test('lc21-合并两个有序链表', () => {
    let l1 = generateListNodes([1, 3, 4])
    let l2 = generateListNodes([1, 2, 5])
    expect(generateListNodes([1,1,2,3,4,5])).toEqual(mergeTwoLists(l1, l2))
})