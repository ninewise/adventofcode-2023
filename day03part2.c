#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define CAP 10

struct range {
	int start;
	int end;
	int number;
};

struct numbers {
	int length;
	int capacity;
	struct range * items;
};

struct symbols {
	int length;
	int capacity;
	int * items;
};

int read_line(FILE * file, struct numbers * ns, struct symbols * ss) {
	char c;
	int i = 0;
	bool in_number = false;
	while ((c = getc(file)) != EOF) {
		switch (c) {
		case '0': case '1': case '2': case '3': case '4':
		case '5': case '6': case '7': case '8': case '9':
			if (!in_number) {
				if (ns->length == ns->capacity) {
					ns->capacity *= 2;
					ns->items = realloc(ns->items, ns->capacity * sizeof(struct range));
				}
				ns->items[ns->length].start = i;
				ns->items[ns->length].number = c - '0';
				ns->length++;
				in_number = true;
			} else {
				ns->items[ns->length - 1].number = 10 * ns->items[ns->length - 1].number + (c - '0');
			}
			break;
		case '*':
			if (ss->length == ss->capacity) {
				ss->capacity *= 2;
				ss->items = realloc(ss->items, ss->capacity * sizeof(int));
			}
			ss->items[ss->length] = i;
			ss->length++;
			// NO break
		case '.': case '\n':
			if (in_number) {
				ns->items[ns->length - 1].end = i;
				in_number = false;
			}
			if (c == '\n') return 0;
			break;
		default:
			// ignore
		}
		i++;
	}
	return EOF;
}

int sum_gears(struct symbols ss, struct numbers ns2, struct numbers ns1, struct numbers ns0) {
	int sum = 0;
	int gear1 = 0;
	int gear2 = 0;
	int two_or_less = 1;
	int ssi, nsi;
	for (ssi = 0; ssi < ss.length; ssi++) {
		for (nsi = 0; nsi < ns2.length; nsi++) {
			if (ss.items[ssi] >= ns2.items[nsi].start - 1 && ss.items[ssi] < ns2.items[nsi].end + 1) {
				if (gear1 == 0) gear1 = ns2.items[nsi].number;
				else if (gear2 == 0) gear2 = ns2.items[nsi].number;
				else two_or_less = 0;
			}
		}
		for (nsi = 0; nsi < ns1.length; nsi++) {
			if (ss.items[ssi] >= ns1.items[nsi].start - 1 && ss.items[ssi] < ns1.items[nsi].end + 1) {
				if (gear1 == 0) gear1 = ns1.items[nsi].number;
				else if (gear2 == 0) gear2 = ns1.items[nsi].number;
				else two_or_less = 0;
			}
		}
		for (nsi = 0; nsi < ns0.length; nsi++) {
			if (ss.items[ssi] >= ns0.items[nsi].start - 1 && ss.items[ssi] < ns0.items[nsi].end + 1) {
				if (gear1 == 0) gear1 = ns0.items[nsi].number;
				else if (gear2 == 0) gear2 = ns0.items[nsi].number;
				else two_or_less = 0;
			}
		}
		sum += gear1 * gear2 * two_or_less;
		gear1 = 0;
		gear2 = 0;
		two_or_less = 1;
	}
	return sum;
}

int main(int argc, char ** argv) {
	FILE * f = fopen(argv[3], "r");
	struct numbers ns2 = { 0, CAP, malloc(CAP * sizeof(struct range)) };
	struct numbers ns1 = { 0, CAP, malloc(CAP * sizeof(struct range)) };
	struct numbers ns0 = { 0, CAP, malloc(CAP * sizeof(struct range)) };
	struct numbers nstemp;
	struct symbols ss2 = { 0, CAP, malloc(CAP * sizeof(int)) };
	struct symbols ss1 = { 0, CAP, malloc(CAP * sizeof(int)) };
	struct symbols ss0 = { 0, CAP, malloc(CAP * sizeof(int)) };
	struct symbols sstemp;
	int sum = 0;
	read_line(f, &ns2, &ss2);
	read_line(f, &ns1, &ss1);
	sum += sum_gears(ss2, ns0 /* empty */, ns2, ns1);
	while (read_line(f, &ns0, &ss0) != EOF) {
		sum += sum_gears(ss1, ns2, ns1, ns0);
		nstemp = ns2;
		ns2 = ns1;
		ns1 = ns0;
		ns0 = nstemp;
		ns0.length = 0;
		sstemp = ss2;
		ss2 = ss1;
		ss1 = ss0;
		ss0 = sstemp;
		ss0.length = 0;
	}
	ns2.length = 0;
	sum += sum_gears(ss0, ns1, ns0, ns2 /* empty */);
	free(ns2.items);
	free(ns1.items);
	free(ns0.items);
	free(ss2.items);
	free(ss1.items);
	free(ss0.items);
	fclose(f);
	printf("%d\n", sum);
	return 0;
}
