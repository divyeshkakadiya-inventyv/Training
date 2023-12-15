import java.util.Scanner;

// 1! 3! 5! 7! 9!....
public class series3 {
    public static void main(String[] args) {        
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int i = 1 , j = 1 , fact = 1;
        while(i <= n*2){
            while(j <= i){
                fact = fact * j;
                j++;
            }
            System.out.print(fact + " ");
            i = i+2;
        }
        sc.close();
    }
}
