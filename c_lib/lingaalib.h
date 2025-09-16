#ifndef COUNT_SECTORS_H
#define COUNT_SECTORS_H

#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

bool IsRunningAsAdmin(void);
bool RelaunchElevated(int argc, char *argv[]);
void CountPhysicalDriveSectors(void);

#ifdef __cplusplus
}
#endif

#endif // COUNT_SECTORS_H
