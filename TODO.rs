

// TODO: I need you to do through test for the method api/spot/create_order.rs
//  I need you to test different combinations of parameters to make sure that all parameters works correctly.
//  Also, we have to be sure that different popular combinations of parameters work as expected.
//  For example a combination of parameters for placing Market order. Try auto-borrow/repay and etc.
//  All parameters should be tested with the real actions acceptable by the exchange.

// TODO: ureq -> use latest. Error

// Read the API doc for spot carefully https://www.gate.com/docs/developers/apiv4/#spot
// Make a list of methods that is not implemented in my library. ONLY SPOT FOR NOW.
// Check carefully.
// Make a plan on what to add. Adding each new method must include:
//  1. making example in examples/sync
//  2. run the example to test it with real data and make sure that it's buildable and there's
//      no "INVALID SIGNATURE" response from the exchange
//  3. when adding new method follow my code style, structure, existing architecture, and keep it consistent.
// Write down the final plan into the file gateio-rs/plan_for_new_methods.md. It should be clear for another developer.

// you need to read this doc carefully https://www.gate.com/docs/developers/apiv4/#retrieve-running-auto-order-list Parameters and python code samples, and implement this method in api/spot/
// Research body parameters table to see how to implement this correctly
// Follow the existing code structure, architecture and principals. It should be consistent. Think harder
// Don't forget to add example in examples/sync and test it. It should return the exchange response. Maybe the request will be rejected, but it should not be "INVALID SIGNATURE" error. Check it.
//
// Do same for 2 more methods:
// https://www.gate.com/docs/developers/apiv4/#cancel-all-price-triggered-orders
// and
// https://www.gate.com/docs/developers/apiv4/#get-a-price-triggered-order
// and
// https://www.gate.com/docs/developers/apiv4/#cancel-a-price-triggered-order

// read the implementation plan carefully gateio-rs/plan_for_new_methods.md. Then implement Methods 4-6 from this plan and follow recommendations