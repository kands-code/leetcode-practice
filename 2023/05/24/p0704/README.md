# p0704 二分查找

给定一个  `n`  个元素有序的（升序）整型数组  `nums` 和一个目标值  `target`，
写一个函数搜索  `nums`  中的 `target`，如果目标值存在返回下标，否则返回 -1

## 示例 1

```text
输入: nums = [-1,0,3,5,9,12], target = 9
输出: 4
解释: 9 出现在 nums 中并且下标为 4
```

## 示例 2

```text
输入: nums = [-1,0,3,5,9,12], target = 2
输出: -1
解释: 2 不存在 nums 中因此返回 -1
```

## 提示

- 你可以假设 `nums` 中的所有元素是不重复的
- `n` 将在 $[1, 10000]$ 之间
- `nums` 的每个元素都将在 $[-9999, 9999]$ 之间

## 解答

这个是一个典型的二分查找问题，由提示可知，默认所给的数组是已经排序好的，我们无需再排序，
而且是升序排列，所以查找顺序就很清楚了

给出 `target`，然后找到范围的中值，然后 `target` 和 中值 进行比较来判断，
根据判断结果来决定是否需要继续划分，或者返回下标，或返回不存在
