#!/bin/sh

echo "stop ccm chain node session cluster every 20s"
#set -x   #print every run cmd

echo "check RUST_MIN_STACK: $RUST_MIN_STACK"

#export RUST_LOG="info"

Stop(){
  pid_file=$1   #get first arg from cmd line
  pgrep -F $pid_file
  pid_is_stale=$?       # get the pgrep return status
  old_pid=$( cat $pid_file )
  echo "pgrep check on pid in existing $pid_file file returned status:$pid_is_stale"
  if [ $pid_is_stale -gt 0 ]; then
      echo "PID $old_pid is stale. Removing file and continuing..."
      rm $pid_file
      return 1
  else
      echo "PID $old_pid is running (or maybe pgrep check errored). Try kill process"
      kill $old_pid
      return $?     #pass kill result to caller
  fi
  return 2
}

Stop "node1.pid"
Stop "node2.pid"
Stop "node3.pid"
Stop "node4.pid"

echo "DONE"

exit 0