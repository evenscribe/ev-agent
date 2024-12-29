# Ev-Agent

Ev-Agent is an always-on monitoring agent inspired by the DataDog agent architecture, designed to collect and transmit various observability signals including logs, profiles, and application traces to Evenscribe servers.

## Status

⚠️ **Note**: This project is currently paused as development efforts are focused on EvProfiler. However, Ev-Agent is planned to be a central component in the future observability stack.

## Overview

Ev-Agent serves as a unified collector for different types of observability data, providing seamless integration with the Evenscribe ecosystem. The agent is designed to be lightweight, reliable, and efficient in resource usage while maintaining continuous monitoring capabilities.

## Current Features

### Log Collection
- **UNIX Socket Support**: Direct log ingestion through UNIX sockets
- **File Watching**: Real-time monitoring of log files with automatic updates
- **Efficient Transport**: Optimized data transmission to Evenscribe servers

## Planned Features

- **Profiling Integration**: 
  - Integration with EvProfiler
  - Continuous profile collection and transmission

- **Application Tracing**:
  - Distributed tracing support
  - Trace context propagation
  - Span collection and aggregation

- **Metrics Collection**:
  - System metrics gathering
  - Custom metric support
  - Automatic aggregation and transmission

- **Enhanced Transport**:
  - Buffer management
  - Retry mechanisms
  - Data compression

## Architecture

### Current Implementation
- Lightweight agent process
- File system monitoring subsystem
- UNIX socket communication layer
- Basic data transport system

### Future Architecture
- Modular collector system
- Plugin-based extension mechanism
- Robust buffering and queuing
- Advanced reliability features

## Technical Details

### Log Collection
- Uses UNIX sockets for efficient local log ingestion
- Implements file watching for traditional log file monitoring
- Supports real-time log streaming


## Related Projects

- [EvProfiler](https://github.com/evenscribe/evprofiler): Continuous profiling system
- [Evenscribe](https://github.com/evenscribe/evenscribe-collector): Server-side components

## Roadmap

1. Complete EvProfiler integration
2. Implement trace collection
3. Add metrics collection
4. Enhance transport layer
5. Add plugin system
