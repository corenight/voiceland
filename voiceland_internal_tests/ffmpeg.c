// This is just a test to view LibAVcodec library

#include <libavcodec/codec.h>
#include <libavcodec/codec_id.h>
#include <stdio.h>

int main() {

  int codec = AV_CODEC_ID_OPUS;

  char *manin = "libopus";

  const AVCodec *asd = avcodec_find_encoder_by_name(manin);

  if (asd == NULL) {
    printf("manin\n");
  } else {
    printf("%s\n", asd->long_name);
  }

  return 0;
}