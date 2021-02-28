// This file was generated by gir (https://github.com/gtk-rs/gir @ a9fa404)
// from gir-files (https://github.com/gtk-rs/gir-files @ 6d6438b1)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) GST_AGGREGATOR_START_TIME_SELECTION_FIRST);
    PRINT_CONSTANT((gint) GST_AGGREGATOR_START_TIME_SELECTION_SET);
    PRINT_CONSTANT((gint) GST_AGGREGATOR_START_TIME_SELECTION_ZERO);
    PRINT_CONSTANT(GST_BASE_PARSE_FLAG_DRAINING);
    PRINT_CONSTANT(GST_BASE_PARSE_FLAG_LOST_SYNC);
    PRINT_CONSTANT((guint) GST_BASE_PARSE_FRAME_FLAG_CLIP);
    PRINT_CONSTANT((guint) GST_BASE_PARSE_FRAME_FLAG_DROP);
    PRINT_CONSTANT((guint) GST_BASE_PARSE_FRAME_FLAG_NEW_FRAME);
    PRINT_CONSTANT((guint) GST_BASE_PARSE_FRAME_FLAG_NONE);
    PRINT_CONSTANT((guint) GST_BASE_PARSE_FRAME_FLAG_NO_FRAME);
    PRINT_CONSTANT((guint) GST_BASE_PARSE_FRAME_FLAG_QUEUE);
    PRINT_CONSTANT((guint) GST_BASE_SRC_FLAG_LAST);
    PRINT_CONSTANT((guint) GST_BASE_SRC_FLAG_STARTED);
    PRINT_CONSTANT((guint) GST_BASE_SRC_FLAG_STARTING);
    PRINT_CONSTANT(GST_BASE_TRANSFORM_SINK_NAME);
    PRINT_CONSTANT(GST_BASE_TRANSFORM_SRC_NAME);
    PRINT_CONSTANT((guint) GST_COLLECT_PADS_STATE_EOS);
    PRINT_CONSTANT((guint) GST_COLLECT_PADS_STATE_FLUSHING);
    PRINT_CONSTANT((guint) GST_COLLECT_PADS_STATE_LOCKED);
    PRINT_CONSTANT((guint) GST_COLLECT_PADS_STATE_NEW_SEGMENT);
    PRINT_CONSTANT((guint) GST_COLLECT_PADS_STATE_WAITING);
    return 0;
}
