import { generateListNodes } from './utils'
import { reverseList } from './lc206'

test('lc206-反转链表', () => {
    expect(generateListNodes([3, 2, 1])).toEqual(reverseList(generateListNodes([1, 2, 3])))
    expect(generateListNodes([1])).toEqual(reverseList(generateListNodes([1])))
})