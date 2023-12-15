import java.util.Scanner;

public class MaxOut2Variable {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int x = sc.nextInt();
        int y = sc.nextInt();
        if(x<=y){
            System.out.println("y is big");
        }else{
            System.out.println("x is big");
        }
    }   
}
