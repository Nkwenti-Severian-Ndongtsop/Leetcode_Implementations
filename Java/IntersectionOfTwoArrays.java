import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

public class IntersectionOfTwoArrays {

    public static int[] intersection(int[] nums1, int[] nums2) {

        Set<Integer> set = new HashSet<>();
        Set<Integer> resultSet = new HashSet<>();

        for (int num: nums1) {
            set.add(num);
        }

        for (int num: nums2) {
            if (set.contains(num)) {
                resultSet.add(num);
            }
        }

        int[] output = new int[resultSet.size()];
        int index = 0;
        for (int val: resultSet) {
            output[index++] = val;
        }
        return output;
    }
    public static void main(String[] args) {
        int[] num1 = {1,2,2,1};
        int[] num2 = {2,2};
        System.out.println(Arrays.toString(IntersectionOfTwoArrays.intersection(num1, num2)));
    }
}
