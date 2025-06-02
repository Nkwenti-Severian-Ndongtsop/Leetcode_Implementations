import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Intersect {

    public static int[] intersect(int[] nums1, int[] nums2) {

        List<Integer> list = new ArrayList<>();
        int range = Math.min(nums1.length, nums2.length);
        int index = 0;
        for (int i = 0; i < range; i++) {
            
            for (int j = 0; j < range; j++) {
                
                if (nums1[i] == nums2[j]) {
                    list.add(nums1[i]);
                }
            }
        }
        int[] result = new int[list.size()];

        for (int val : result) {
            result[index] = val;
        }
        return result;
    }

    public static void main(String[] args) {
        int[] nums1 = { 1, 2, 2, 1 };
        int[] nums2 = { 2, 2 };
        System.out.println(Arrays.toString(Intersect.intersect(nums1, nums2)));
    }
}
