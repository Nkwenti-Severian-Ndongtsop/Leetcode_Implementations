import java.util.HashMap;
import java.util.Map;

public class RomanToInteger {

    static public int romanToInt(String s) {//DXIX

        Map<String, Integer> map = new HashMap<>();
        map.put("I", 1);
        map.put("V", 5);
        map.put("X", 10);
        map.put("L", 50);
        map.put("C", 100);
        map.put("D", 500);
        map.put("M", 1000);

        int prev = 0;
        int total = 0;

        for (int i=s.length() - 1; i>=0; i--) {
            int curr = map.get(String.valueOf(s.charAt(i)));

            if (curr < prev) {
                total -= curr;
            } else {
                total += curr;
            }
            prev = curr;
        }
        return total;
    }
    public static void main(String[] args) {
        String s = "MD";
        System.out.println(RomanToInteger.romanToInt(s));
    }
}