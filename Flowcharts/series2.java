import java.util.Scanner;

// 112123211234321....
public class series2 {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        for(int i = 1 ; i <= n ; i++){
            for(int j = 1 ; j <= i ; j++){
                System.out.print(j + " ");
            }
            for(int j = i-1 ; j >= 1 ; j--){
                System.out.print(j + " ");
            }
        }
    }
}
