import java.util.Scanner;

class RadiusToArea{
    public static void main(){
        Scanner sc = new Scanner(System.in);
        System.out.println("Enter Radius : ");
        int radius = sc.nextInt();
        System.out.println("Your Area is : " + radius * radius * 3.14);
        sc.close();
    }
}