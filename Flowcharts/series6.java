// 1
// 232
// 34543
// 4567654
// 567898765
// 67890109876
// 7890123210987 

public class series6{
    public static void main(String[] args) {
      int i = 1 , n = 7 , j = 1;
      for(i=1 ; i<=n ; i++){
        for(j=i ; j<(i*2-1) ; j++){
          System.out.print(j%10);
        }
        for(j=(i*2-1) ; j>(i-1) ; j--){
          System.out.print(j%10);
        }
        System.out.println();
      }
  }
}