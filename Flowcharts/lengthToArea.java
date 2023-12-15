import java.util.Scanner;

class lengthToArea{
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int length = sc.nextInt();
        int breath = sc.nextInt();

        System.out.println("Area is : " + length * breath);
        System.out.println("Parimeter is :" + (length * breath) * 2);
        sc.close();
    
    }
}