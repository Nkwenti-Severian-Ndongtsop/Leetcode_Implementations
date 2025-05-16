import java.util.LinkedHashMap;
import java.util.Map;

public class FirstUniqueCha {
    
    static public int firstUniqChar(String s) { // Input: s = "leetcode"
        
        Map<Character, Integer> map = new LinkedHashMap<>();

        for (char ch : s.toCharArray()) {
            map.put(ch, map.getOrDefault(ch, 0) + 1);
        }

        for (Map.Entry<Character, Integer> entry : map.entrySet()) {
            if (entry.getValue() == 1) {
                return s.indexOf(entry.getKey());
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        System.out.println(FirstUniqueCha.firstUniqChar("leetcode"));
    }
    
}
