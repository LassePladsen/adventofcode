#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
  int *arr;
  size_t size;
} IntArr;

typedef struct {
  IntArr col1;
  IntArr col2;
} Columns;

Columns read_input_cols(char filename[]) {
  FILE *fptr;
  fptr = fopen(filename, "r");
  if (fptr == NULL) {
    printf("Could not open file: %s\n", filename);
    exit(1);
  }
  int MAXLEN = 50;
  char line[MAXLEN];
  char *split;

  size_t size1 = 0, size2 = 0;
  size_t capacity1 = 10, capacity2 = 10;
  int *col1 = malloc(capacity1 * sizeof(int));
  int *col2 = malloc(capacity2 * sizeof(int));
  int val1, val2;

  // Loop over lines
  while (fgets(line, MAXLEN, fptr)) {
    // Split into columns by whitespace, and add to correct column
    split = strtok(line, " ");
    if (split == NULL)
      continue;

    // Parse as integer
    val1 = atoi(split);

    // Resize if necessary
    if (size1 >= capacity1) {
      capacity1 *= 2;
      col1 = realloc(col1, capacity1 * sizeof(int));
    }
    col1[size1++] = val1;

    split = strtok(NULL, " ");
    if (split == NULL)
      continue;

    // Parse as integer
    val2 = atoi(split);

    // Resize if necessary
    if (size2 >= capacity2) {
      capacity2 *= 2;
      col2 = realloc(col2, capacity2 * sizeof(int));
    }
    col2[size2++] = val2;
  }
  fclose(fptr);

  // Create char arrays
  IntArr *c1 = malloc(sizeof(IntArr));
  *c1 = (IntArr){col1, size1};
  IntArr *c2 = malloc(sizeof(IntArr));
  *c2 = (IntArr){col2, size2};

  return (Columns){*c1, *c2};
}

void print_col_vals(IntArr col) {
  for (int i = 0; i < col.size - 1; i++) {
    printf("%i,", col.arr[i]);
  }
  // Print last without comma
  printf("%i", col.arr[col.size - 1]);
  printf("\n");
}

void free_cols(Columns cols) {
  free(cols.col1.arr);
  free(cols.col2.arr);
}

/// Custom comparison function for integers (ascending order)
int cmp_ints(const void *a, const void *b) { return (*(int *)a - *(int *)b); }
void sort_cols(Columns cols) {
  qsort(cols.col1.arr, cols.col1.size, sizeof(int), cmp_ints);
  qsort(cols.col2.arr, cols.col2.size, sizeof(int), cmp_ints);
}

/// Calculate sum of diffs between each element in the columns
int column_diffs(Columns cols) {
  int sum = 0;
  for (int i = 0; i < cols.col1.size; i++) {
    sum += abs(cols.col1.arr[i] - cols.col2.arr[i]);
  }
  return sum;
}

int main() {
  Columns cols = read_input_cols("../input");
  printf("Length of columns: %lu\n", cols.col1.size);

  // print_col_vals(cols.col1);
  // print_col_vals(cols.col2);
  sort_cols(cols);

  int tot_diff = column_diffs(cols);
  printf("Sum of differences between columns: %i\n", tot_diff);

  free_cols(cols);
  return 0;
}
