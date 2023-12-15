import java.util.Scanner;

public class AgeValidation {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        System.out.println("Enter Age : ");
        int age = sc.nextInt();
        if(age>=21)
            System.out.println("Adult");
        else
            System.out.println("Not Adult"); 
        sc.close();
    }
}

