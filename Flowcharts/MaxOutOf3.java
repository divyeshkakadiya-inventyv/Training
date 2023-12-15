import java.util.Scanner;

public class MaxOutOf3 {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int x = sc.nextInt();
        int y = sc.nextInt();
        int z = sc.nextInt();

        if(x >= y){
            if(x >= z){
                System.out.println("x is big");
            }else{
                System.out.println("z is big");
            }
        }else{
            if(y >= z){
                System.out.println("y is big");
            }
        }
    }
}
