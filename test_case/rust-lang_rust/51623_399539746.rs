c
void simple_dispatch() {
// Initialize the runtime
hsa_init();

// Retrieve the kernel agent
hsa_agent_t kernel_agent;
hsa_iterate_agents(get_kernel_agent,   &kernel_agent);

// Create a queue in the kernel agent. The queue can hold 4 packets, 
// and has no callback or service queue associated with it
hsa_queue_t *queue;

hsa_queue_create(kernel_agent, 4, HSA_QUEUE_TYPE_SINGLE, NULL, NULL, 0, 0, &queue);

// Since no packets have been enqueued yet, we use zero as 
// the packet ID and bump the write index accordingly
hsa_queue_add_write_index_relaxed(queue, 1);

uint64_t packet_id =0;
// Calculate the virtual address where to place the packet
hsa_kernel_dispatch_packet_t* packet = 
  (hsa_kernel_dispatch_packet_t*) queue->base_address + packet_id;

// Populate fields in kernel dispatch packet, except for the header, 
// the setup, and the completion signal fields
initialize_packet(packet);

// Create a signal with an initial value of one to monitor the task completion
hsa_signal_create(1, 0, NULL, &packet->completion_signal);

// Notify the queue that the packet is ready to be processed
packet_store_release((uint32_t*) packet, header(HSA_PACKET_TYPE_KERNEL_DISPATCH), 
                                    kernel_dispatch_setup());

hsa_signal_store_screlease(queue->doorbell_signal,    packet_id);

// Wait for the task to finish, which is the same as waiting for
// the value of the completion signal to be zero
while (hsa_signal_wait_scacquire(packet->completion_signal, HSA_SIGNAL_CONDITION_EQ,
                                 0, UINT64_MAX, HSA_WAIT_STATE_ACTIVE) != 0);

// Done! The kernel has completed. Time to cleanup resources and leave
hsa_signal_destroy(packet->completion_signal);
hsa_queue_destroy(queue);
hsa_shut_down();
}
