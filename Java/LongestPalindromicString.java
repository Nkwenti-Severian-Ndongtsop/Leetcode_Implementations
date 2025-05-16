import java.util.HashMap;
import java.util.Map;

public class LongestPalindromicString {
    /**
     * Input: s = "babad"
     * Output: "bab"
     * 
     * @param s
     * @return
     */

    static public String longestPalindrome(String s) {

        Map<Integer, String> listMap = new HashMap<>();

        for (int i = 0; i<s.length(); i++) {

            for (int j = i; j<s.length(); j++) {
                String testString = new String(s.subSequence(i, j+1).toString());

                if ( new StringBuilder(testString).reverse().toString().equals(testString)) {
                    listMap.put(testString.length(), testString);
                }
            }
        }
        
        int tempMax = 0;
        
        for (Integer key : listMap.keySet()) {

            if (key > tempMax ) {
                tempMax = key;
            }
        }

        return listMap.get(tempMax);
    }

    public static void main(String[] args) {
        
        System.out.println(LongestPalindromicString.longestPalindrome("babad"));

    }
}
