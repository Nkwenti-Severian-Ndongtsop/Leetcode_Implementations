public class ValidAnagram {
    
    public boolean isAnagram(String s, String t) {
        
        if ( s.length() != t.length() ) {
            return false;
        }

        for ( int i=0; i<s.length(); i++) {

            if (! s.contains(String.valueOf(t.charAt(i)))) {
                return false;
            }
        }
        return true;
    }
}
