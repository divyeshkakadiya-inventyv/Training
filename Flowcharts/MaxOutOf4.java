import java.util.Scanner;

public class MaxOutOf4 {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
    int x = sc.nextInt();
    int y = sc.nextInt();
    int z = sc.nextInt();
    int w = sc.nextInt();
    
    if(x >= y){
        if(x >= z){
            if(x >= w){
                System.out.println("x is big");
            }else{
                System.out.println("W is big");
            }
        }else{
            if(z >= w){
                System.out.println("z is big");
            }
        }
    }else{
        if(y>=z){
            if(y>=w){
                System.out.println("y is big");
            }else{
                System.out.println("w is big");
            }
        }else{
            if(z >= w){
                System.out.println("z is big");
            }else{
                System.out.println("w is big");
            }
        }
        sc.close();
    }
        
    }
}
