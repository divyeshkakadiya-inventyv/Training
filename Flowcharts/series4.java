import java.util.Scanner;

// 1 -2 3 -4 5 -6....
public class series4 {
    public static void main(String[] args) {
        int i=1,s=1,n;
        Scanner sc=new Scanner(System.in);

        System.out.print("Enter nth term ");
        n=sc.nextInt();

        while(i<=n)
        {
            System.out.print(i*s + " ");
            s=s*-1;		// changing the sign
            i=i+1;
        }
        sc.close();
    }
}
