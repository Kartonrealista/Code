#include <stdio.h>
#include <stdlib.h>

void draw(char x[3][3])
{
    int i, j;
    for(i = 0; i < 3; i++)
    {
        for(j = 0; j < 3; j++)
        {
            printf("%c ", x[i][j]);
        }
        printf("\n");
    }
    printf("\n");
}

void wincheck(char x[3][3])
{
  int k, l;
    for(k = 0; k < 3; k++)
    {
        int vo = 0, ho = 0, vx = 0, hx = 0, dbx = 0, dbo = 0, dfx = 0, dfo = 0;
        for(l = 0; l < 3; l++)
        {
            if(x[k][l] == 'x')
            {
                vx++;
            }
            else if(x[k][l] == 'o')
            {
                vo++;
            }
            if(x[l][k] == 'x')
            {
                hx++;
            }
            else if(x[l][k] == 'o')
            {
                ho++;
            }
            if(x[l][l] == 'x')
            {
                dbx++;
            }
            else if(x[l][l] == 'o')
            {
                dbo++;
            }
            if(x[l][2-l] == 'x')
            {
                dfx++;
            }
            else if(x[l][2-l] == 'o')
            {
                dfo++;
            }


        }
        if(vx == 3 || hx ==3 || dbx == 3 || dfx == 3)
        {
            printf("X wins\n");
            exit(0);
        }
        else if(vo == 3 || ho ==3 || dbo == 3 || dfo == 3)
        {
            printf("O wins\n");
            exit(0);
        }
    }  
}

void move(char x[3][3], char y)
{
    int input1, input2;
    scanf("%d %d", &input1, &input2);
    if(x[input1][input2] == '_')
    {
        x[input1][input2] = y;
        draw(x);
    }
    else
    {
        draw(x);
        move(x, y);
    }
    wincheck(x);
}

int main()
{
    char arr[3][3] = {
        {
            '_', '_', '_'
        },
        {
            '_', '_', '_'
        },
        {
            '_', '_', '_'
        }

    };
    char x = 'x';
    char o = 'o';
    draw(arr);

    move(arr, x);
    int m;
    for(m = 0; m < 4; m++)
    {
        move(arr, o);
        move(arr, x);
    }
    printf("draw\n");
    
    return 0;
}
