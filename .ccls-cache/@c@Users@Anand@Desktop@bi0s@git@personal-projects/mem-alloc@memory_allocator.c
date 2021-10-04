#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <unistd.h>
#include <string.h>

#define ALIGNMENT 8 //8-byte alignment
#define ALIGN(size) (((size) + (ALIGNMENT-1)) & ~0x7)
#define header_size ALIGN(sizeof(size_t))
size_t *heap_end;
size_t *heap_start;
int flag = 0;

void split(size_t* block,size_t size) //Only allocating amount of space requested by malloc
{
    size_t curr_size = *block;
    curr_size -= size;
    *block = size;
    block = (char *)block + size;
    *block = curr_size;
}

size_t *get_block(size_t size) 
{
    size_t *current = heap_start;
    while (current <= heap_end) //Traverse through heap to search for available chunks of requested size
    {
        if(*current == 0)
            break;
        else if ((!(*current & 1) && *current >= size))
        {
            split(current,size); //Splitting blocks, basically only allocate requested size, split the other part into a new chunk
            return current;
        }
        else if (!(*current & 1))
        {
            size_t *next;
            next = (char *)current + *current;
            if(!(*next & 1)) //Some basic coalescing -> See if next block is also free
            {
                *current += *next; //Add the size of next block to current block
                *next = 0; //Remove the next block
                if (*current >= size) //If coalescing gives us the memory we need, then split and return that memory
                {
                    split(current,size); //Splitting blocks
                    return current;
                }
            }
        }
        current = (char *)current + (*current & ~1L); //Move on to the next chunk in the heap
    }
    return NULL;
}

size_t *my_malloc(size_t size)
{
    size_t blocksize = ALIGN(size + header_size); //Align (requested size + 8 bytes(for header)) to the nearest 8 bytes
    size_t *header = get_block(blocksize); //See if a previously freed block is available in memory
    if(header != NULL)
    {
        *header = *header | 1; //Mark the block as in use (last bit of header = 1)
    }
    else //Only allocate new memory if required
    {
        header = sbrk(blocksize); //Request the required memory
        *header = blocksize | 1; //Mark the block as in use
        if(flag == 0)
        {
            heap_start = header; //Mark the start of our heap as our first allocation
            heap_end = (char *)heap_start + blocksize; //Initialize heap_end
            flag = 1;
        }
        else
        {
            heap_end = (char *)heap_end + blocksize; //Increment the heap_end pointer, to show the end of the current heap
        }
    }
    return (char *)header + header_size; //Return pointer to memory following the header
}

void my_free(size_t *ptr)
{
    size_t *header = (char *)ptr - header_size; //Get the header, not the allocated memory
    *header = *header & ~1L; // Unmark allocated bit
}

int init(void)
{
    heap_end = heap_start = sbrk(0); //Initialise addresses for heap start and end (Beginning of heap)
    return 0;
}

int main()
{
    init();
    printf("\t----Starting memory allocator----\n");
    size_t *a = my_malloc(0x20);
    printf("Address of allocation 1 : %p\n", a);
    size_t *b = my_malloc(0x50);
    printf("Address of allocation 2 : %p\n", b);
    size_t *c = my_malloc(0x30);
    printf("Address of allocation 3 : %p\n", c);
    my_free(a);
    my_free(b);
    size_t *d = my_malloc(0x70); //Shows coalescing of first 2 free blocks
    printf("Address of allocation 4 (Coalescing): %p\n", d);
    printf("\n");
    printf("Want to try it yourself? Here you go!\n");
    size_t *array[10];
    int check = 0;
    size_t size;
    int index;
    do
    {
        int choice;
        printf("Enter 1 to malloc, 2 to free, or any other number to exit!\n");
        scanf("%d", &choice);
        switch(choice)
        {
            case 1:
                printf("\nEnter the index (0-9)\n");
                scanf("%d", &index);
                if(!array[index] && index >= 0 && index < 10)
                {
                    printf("\nEnter the size to malloc\n");
                    scanf("%d", &size);
                    array[index] = my_malloc(size);
                    printf("Address of allocation : %p\n", array[index]);
                }
                else
                    printf("The index is either invalid, or in use!\n");
                break;
            case 2:
                printf("\nEnter the index (0-9)\n");
                scanf("%d", &index);
                if(array[index] && index >= 0 && index < 10)
                {
                    my_free(array[index]);
                    array[index] = NULL;
                    printf("Free complete!\n");
                }
                else
                    printf("The index is either invalid, or not in use!\n");
                break;
            default:
                printf("Bye!\n");
                check = 1;
        }
        printf("\n");
    } 
    while (check == 0);
    return 0;
}