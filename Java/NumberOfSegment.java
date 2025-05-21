import java.util.ArrayList;
import java.util.List;

public class NumberOfSegment {

    public int countSegments(String s) {

        int indexLastWord = s.lastIndexOf(' ');
        List<String> str = new ArrayList<>();
        str.add(s.substring(indexLastWord, s.length() - 1));
        int prevIndex = 0;
        for (int i = 0; i < s.length(); i++) {
            if (s.charAt(i) == ' ') {
                str.add(s.substring(prevIndex, i));
                prevIndex = i + 1;
            }

        }
        return str.size();
    }

}
