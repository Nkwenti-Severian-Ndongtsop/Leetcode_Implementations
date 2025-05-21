import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.TreeSet;

public class ThirdMaximum {
    
    public int thirdMax(int[] nums) {
        
        Set<Integer> set = new TreeSet<>();
        for (int val : nums) {
            set.add(val);
        }
        List<Integer> list = new ArrayList<>(set);
        if ( list.size() == 3 ) {
            return list.get(0);
        } else if ( list.size() == 2 ) {
            return list.get(1);
        } else if ( list.size() == 1 ) {
            return list.get(0);
        }
        return list.get(2);
    }
    
}
