import {maxDepth} from './of55-1'

test('of55-1-二叉树的最大深度', () => {
    let t = {
        left: {
            left: {
                val: 2,
                left: null,
                right: null
            },
            right: null,
            val: 4
        },
        right: {
            left: null,
            right: {
                val: 6,
                left: {
                    val: 7,
                    left: null,
                    right: null
                },
                right: null
            },
            val: 5
        },
        val: 1
    }
    expect(4).toEqual(maxDepth(t))
})