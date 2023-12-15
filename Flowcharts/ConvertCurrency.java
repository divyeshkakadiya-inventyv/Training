import java.util.Scanner;

public class ConvertCurrency {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        System.out.println("Enter type (p/r) : ");
        String type = sc.nextLine();
        if(type == "p"){
            int paisa = sc.nextInt();
            System.out.println("Your rs is :" + paisa / 100);
        }else{
            if(type == "r"){
                int res = sc.nextInt();
                System.out.println("Your paisa is :" + res*100);
            }else{
                System.out.println("Invalid Input");
            }
        }
        sc.close();
    }
}
