public class IndexOfFirstOccurence {

    public int strStr(String haystack, String needle) {

        String expectedString = new String();
        for (int i = 0; i < haystack.length() - 1; i++) {
            if (haystack.charAt(i) == needle.charAt(0)) {
                if (haystack.length() - i > 3) {
                    expectedString = haystack.substring(i, i + needle.length() - 1);
                    if (expectedString.equals(needle)) {
                        return i;
                    }
                }
            }
        }
        return -1;
    }
}
