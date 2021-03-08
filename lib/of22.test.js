import { removeNthFromEnd } from './of22'
import { ListNode, generateListNodes} from './utils'

test('of22-删除链表的倒数第N个节点', () => {
    expect(generateListNodes([1, 2, 3, 5])).toEqual(removeNthFromEnd(generateListNodes([1, 2, 3, 4, 5]), 2))
    expect(generateListNodes([2])).toEqual(removeNthFromEnd(generateListNodes([1, 2]), 2))
})