import {deleteNode} from './lc237'
import { ListNode, generateListNodes } from './utils'

test("lc237-删除链表中的节点", () => {
    let l1 = generateListNodes([1,2,3])
    deleteNode(l1.next)
    expect(l1.next).toEqual(new ListNode(3))
} )