public class Mistake {
    public static void main(String[] args){
        StringBuilder letters = new StringBuilder("Hi");
        StringBuilder differentLetters = letters;
        System.out.println(differentLetters);
        letters.append(" there");

        System.out.println(differentLetters);
    }
}