import { ListNode, generateListNodes, TreeNode } from './utils'

test('ListNode create', () => {
    let l1 = new ListNode(1)
    expect(l1.val).toBe(1)
    expect(l1.next).toBe(null)
})

test('ListNodes create by array', () => {
    let l1 = new ListNode(1)
    l1.next = new ListNode(2)
    l1.next.next = new ListNode(3)   
    expect(generateListNodes([1, 2, 3])).toEqual(l1)
})

test('TreeNode create', () => {
    let t1 = new TreeNode(1, new TreeNode(2), new TreeNode(3))
    expect(1).toEqual(t1.val)
    expect(2).toEqual(t1.left.val)
    expect(3).toEqual(t1.right.val)
    expect({
        val: 1,
        left: {val: 2, left: null, right: null},
        right: {val: 3, left: null, right: null,}
    }).toEqual(t1)
})