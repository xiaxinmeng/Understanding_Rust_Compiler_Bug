
AttributeList AL = F->getAttributes();
AL = AL.addAttributes(F->getContext(), Index, B);
F->setAttributes(AL);
