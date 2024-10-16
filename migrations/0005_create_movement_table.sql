CREATE TABLE movements (
    id UUID PRIMARY KEY,
    printer_id UUID REFERENCES printers(id) ON DELETE SET NULL,
    item_id UUID NOT NULL,
    quantity INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
