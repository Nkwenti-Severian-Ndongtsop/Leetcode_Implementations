public class MergeSortedList {

    public static class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    // l1 = 124
    // l2 = 134
    // merged: (1)->(1)->()
    public static ListNode mergeTwoLists(ListNode list1, ListNode list2) {

        ListNode head = new ListNode(-1);
        ListNode currentNode = head;

        while (list1 != null && list2 != null) {

            if (list1.val <= list2.val) {
                currentNode.next = list1;
                list1 = list1.next;
            } else if (list2.val < list1.val) {
                currentNode.next = list2;
                list2 = list2.next;
            }
            currentNode = currentNode.next;

        }

        if ( list1 != null) currentNode.next = list1;
        if ( list2 != null) currentNode.next = list2;

        return head.next;
    }

    static void show(MergeSortedList.ListNode list) {
        
        while ( list.next != null) {
            System.out.printf("%d -> ", list.val);
            list = list.next;
        }
        System.out.printf("%d", list.val);
    }

    public static void main(String[] args) {
        
        MergeSortedList.ListNode list1 = new MergeSortedList.ListNode(1, new MergeSortedList.ListNode(2, new MergeSortedList.ListNode(4, null)));
        MergeSortedList.ListNode list2 = new MergeSortedList.ListNode(1, new MergeSortedList.ListNode(3, new MergeSortedList.ListNode(4, null)));
        MergeSortedList.show(mergeTwoLists(list1, list2));
    }
}
